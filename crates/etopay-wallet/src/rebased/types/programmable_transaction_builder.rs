// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// Modifications Copyright (c) 2025 ETO GRUPPE TECHNOLOGIES GmbH
// SPDX-License-Identifier: Apache-2.0

//! Utility for generating programmable transactions, either by specifying a
//! command or for migrating legacy transactions

use indexmap::IndexMap;
use serde::Serialize;

use super::{
    Argument, CallArg, Command, Identifier, IotaAddress, ObjectArg, ObjectID, ObjectRef, ProgrammableMoveCall,
    ProgrammableTransaction, TypeTag,
};

/// Wrapper for Iota Rebased Errors
#[derive(thiserror::Error, Debug)]
pub enum BuilderError {
    #[error("InvariantViolation: {0}")]
    InvariantViolation(String),

    #[error("LengthMismatch: {0}")]
    LengthMismatch(String),

    #[error("Argument: {0}")]
    Mismatch(String),

    #[error("Bcs: {0}")]
    Bcs(#[from] bcs::Error),
}

#[derive(PartialEq, Eq, Hash)]
enum BuilderArg {
    Object(ObjectID),
    Pure(Vec<u8>),
    ForcedNonUniquePure(usize),
}

#[derive(Default)]
pub struct ProgrammableTransactionBuilder {
    inputs: IndexMap<BuilderArg, CallArg>,
    commands: Vec<Command>,
}

impl ProgrammableTransactionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn finish(self) -> ProgrammableTransaction {
        let Self { inputs, commands } = self;
        let inputs = inputs.into_values().collect();
        ProgrammableTransaction { inputs, commands }
    }

    pub fn pure_bytes(&mut self, bytes: Vec<u8>, force_separate: bool) -> Argument {
        let arg = if force_separate {
            BuilderArg::ForcedNonUniquePure(self.inputs.len())
        } else {
            BuilderArg::Pure(bytes.clone())
        };
        let (i, _) = self.inputs.insert_full(arg, CallArg::Pure(bytes));
        Argument::Input(i as u16)
    }

    pub fn pure<T: Serialize>(&mut self, value: T) -> Result<Argument, BuilderError> {
        Ok(self.pure_bytes(
            bcs::to_bytes(&value)?,
            // force separate
            false,
        ))
    }

    /// Like pure but forces a separate input entry
    pub fn force_separate_pure<T: Serialize>(&mut self, value: T) -> Result<Argument, BuilderError> {
        Ok(self.pure_bytes(
            bcs::to_bytes(&value)?,
            // force separate
            true,
        ))
    }

    pub fn obj(&mut self, obj_arg: ObjectArg) -> Result<Argument, BuilderError> {
        let id = obj_arg.id();
        let obj_arg = if let Some(old_value) = self.inputs.get(&BuilderArg::Object(id)) {
            let old_obj_arg = match old_value {
                CallArg::Pure(_) => {
                    return Err(BuilderError::InvariantViolation("object has pure argument".to_string()));
                }
                CallArg::Object(arg) => arg,
            };
            match (old_obj_arg, obj_arg) {
                (
                    ObjectArg::SharedObject {
                        id: id1,
                        initial_shared_version: v1,
                        mutable: mut1,
                    },
                    ObjectArg::SharedObject {
                        id: id2,
                        initial_shared_version: v2,
                        mutable: mut2,
                    },
                ) if v1 == &v2 => {
                    if !(id1 == &id2 && id == id2) {
                        return Err(BuilderError::InvariantViolation(
                            "object has id does not match call arg".to_string(),
                        ));
                    }
                    ObjectArg::SharedObject {
                        id,
                        initial_shared_version: v2,
                        mutable: *mut1 || mut2,
                    }
                }
                (old_obj_arg, obj_arg) => {
                    if old_obj_arg != &obj_arg {
                        return Err(BuilderError::Mismatch(format!(
                            "Mismatched Object argument kind for object {id}. \
                        {old_value:?} is not compatible with {obj_arg:?}"
                        )));
                    }
                    obj_arg
                }
            }
        } else {
            obj_arg
        };
        let (i, _) = self
            .inputs
            .insert_full(BuilderArg::Object(id), CallArg::Object(obj_arg));
        Ok(Argument::Input(i as u16))
    }

    pub fn input(&mut self, call_arg: CallArg) -> Result<Argument, BuilderError> {
        match call_arg {
            CallArg::Pure(bytes) => Ok(self.pure_bytes(bytes, /* force separate */ false)),
            CallArg::Object(obj) => self.obj(obj),
        }
    }

    pub fn make_obj_vec(&mut self, objs: impl IntoIterator<Item = ObjectArg>) -> Result<Argument, BuilderError> {
        let make_vec_args = objs.into_iter().map(|obj| self.obj(obj)).collect::<Result<_, _>>()?;
        Ok(self.command(Command::MakeMoveVec(None, make_vec_args)))
    }

    pub fn command(&mut self, command: Command) -> Argument {
        let i = self.commands.len();
        self.commands.push(command);
        Argument::Result(i as u16)
    }

    /// Will fail to generate if given an empty ObjVec
    pub fn move_call(
        &mut self,
        package: ObjectID,
        module: Identifier,
        function: Identifier,
        type_arguments: Vec<TypeTag>,
        call_args: Vec<CallArg>,
    ) -> Result<(), BuilderError> {
        let arguments = call_args.into_iter().map(|a| self.input(a)).collect::<Result<_, _>>()?;
        self.command(Command::move_call(package, module, function, type_arguments, arguments));
        Ok(())
    }

    pub fn programmable_move_call(
        &mut self,
        package: ObjectID,
        module: Identifier,
        function: Identifier,
        type_arguments: Vec<TypeTag>,
        arguments: Vec<Argument>,
    ) -> Argument {
        self.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package,
            module,
            function,
            type_arguments,
            arguments,
        })))
    }

    pub fn transfer_arg(&mut self, recipient: IotaAddress, arg: Argument) -> Result<(), BuilderError> {
        self.transfer_args(recipient, vec![arg])
    }

    pub fn transfer_args(&mut self, recipient: IotaAddress, args: Vec<Argument>) -> Result<(), BuilderError> {
        let rec_arg = self.pure(recipient)?;
        self.commands.push(Command::TransferObjects(args, rec_arg));
        Ok(())
    }

    pub fn transfer_object(&mut self, recipient: IotaAddress, object_ref: ObjectRef) -> Result<(), BuilderError> {
        let rec_arg = self.pure(recipient)?;
        let obj_arg = self.obj(ObjectArg::ImmOrOwnedObject(object_ref));
        self.commands.push(Command::TransferObjects(vec![obj_arg?], rec_arg));
        Ok(())
    }

    pub fn transfer_iota(&mut self, recipient: IotaAddress, amount: Option<u64>) -> Result<(), BuilderError> {
        let rec_arg = self.pure(recipient)?;
        let coin_arg = if let Some(amount) = amount {
            let amt_arg = self.pure(amount)?;
            self.command(Command::SplitCoins(Argument::GasCoin, vec![amt_arg]))
        } else {
            Argument::GasCoin
        };
        self.command(Command::TransferObjects(vec![coin_arg], rec_arg));
        Ok(())
    }

    pub fn pay_all_iota(&mut self, recipient: IotaAddress) -> Result<(), BuilderError> {
        let rec_arg = self.pure(recipient)?;
        self.command(Command::TransferObjects(vec![Argument::GasCoin], rec_arg));
        Ok(())
    }

    /// Will fail to generate if recipients and amounts do not have the same
    /// lengths
    pub fn pay_iota(&mut self, recipients: Vec<IotaAddress>, amounts: Vec<u64>) -> Result<(), BuilderError> {
        self.pay_impl(recipients, amounts, Argument::GasCoin)
    }

    /// Will fail to generate if recipients and amounts do not have the same
    /// lengths. Or if coins is empty
    pub fn pay(
        &mut self,
        coins: Vec<ObjectRef>,
        recipients: Vec<IotaAddress>,
        amounts: Vec<u64>,
    ) -> Result<(), BuilderError> {
        let mut coins = coins.into_iter();
        let Some(coin) = coins.next() else {
            return Err(BuilderError::LengthMismatch("coins vector is empty".to_string()));
        };
        let coin_arg = self.obj(ObjectArg::ImmOrOwnedObject(coin))?;
        let merge_args: Vec<_> = coins
            .map(|c| self.obj(ObjectArg::ImmOrOwnedObject(c)))
            .collect::<Result<_, _>>()?;
        if !merge_args.is_empty() {
            self.command(Command::MergeCoins(coin_arg, merge_args));
        }
        self.pay_impl(recipients, amounts, coin_arg)
    }

    fn pay_impl(
        &mut self,
        recipients: Vec<IotaAddress>,
        amounts: Vec<u64>,
        coin: Argument,
    ) -> Result<(), BuilderError> {
        if recipients.len() != amounts.len() {
            return Err(BuilderError::LengthMismatch(format!(
                "Recipients and amounts mismatch. Got {} recipients but {} amounts",
                recipients.len(),
                amounts.len()
            )));
        }
        if amounts.is_empty() {
            return Ok(());
        }

        // collect recipients in the case where they are non-unique in order
        // to minimize the number of transfers that must be performed
        let mut recipient_map: IndexMap<IotaAddress, Vec<usize>> = IndexMap::new();
        let mut amt_args = Vec::with_capacity(recipients.len());
        for (i, (recipient, amount)) in recipients.into_iter().zip(amounts).enumerate() {
            recipient_map.entry(recipient).or_default().push(i);
            amt_args.push(self.pure(amount)?);
        }
        let Argument::Result(split_primary) = self.command(Command::SplitCoins(coin, amt_args)) else {
            panic!("self.command should always give a Argument::Result")
        };
        for (recipient, split_secondaries) in recipient_map {
            let rec_arg = self.pure(recipient)?;
            let coins = split_secondaries
                .into_iter()
                .map(|j| Argument::NestedResult(split_primary, j as u16))
                .collect();
            self.command(Command::TransferObjects(coins, rec_arg));
        }
        Ok(())
    }
}
