(function() {
    var implementors = Object.fromEntries([["etopay_sdk",[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"etopay_sdk/enum.WalletError.html\" title=\"enum etopay_sdk::WalletError\">WalletError</a>&gt; for <a class=\"enum\" href=\"etopay_sdk/error/enum.Error.html\" title=\"enum etopay_sdk::error::Error\">Error</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"etopay_sdk/kdbx/enum.KdbxStorageError.html\" title=\"enum etopay_sdk::kdbx::KdbxStorageError\">KdbxStorageError</a>&gt; for <a class=\"enum\" href=\"etopay_sdk/enum.WalletError.html\" title=\"enum etopay_sdk::WalletError\">WalletError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"etopay_sdk/share/enum.ShareError.html\" title=\"enum etopay_sdk::share::ShareError\">ShareError</a>&gt; for <a class=\"enum\" href=\"etopay_sdk/enum.WalletError.html\" title=\"enum etopay_sdk::WalletError\">WalletError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"etopay_sdk/types/currencies/enum.Currency.html\" title=\"enum etopay_sdk::types::currencies::Currency\">Currency</a>&gt; for ApiCryptoCurrency"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"etopay_sdk/types/error/enum.TypeError.html\" title=\"enum etopay_sdk::types::error::TypeError\">TypeError</a>&gt; for <a class=\"enum\" href=\"etopay_sdk/enum.WalletError.html\" title=\"enum etopay_sdk::WalletError\">WalletError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"etopay_sdk/types/error/enum.TypeError.html\" title=\"enum etopay_sdk::types::error::TypeError\">TypeError</a>&gt; for <a class=\"enum\" href=\"etopay_sdk/error/enum.Error.html\" title=\"enum etopay_sdk::error::Error\">Error</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.1/std/primitive.u64.html\">u64</a>&gt; for <a class=\"struct\" href=\"etopay_sdk/types/currencies/struct.CryptoAmount.html\" title=\"struct etopay_sdk::types::currencies::CryptoAmount\">CryptoAmount</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"etopay_sdk/types/currencies/struct.CryptoAmount.html\" title=\"struct etopay_sdk::types::currencies::CryptoAmount\">CryptoAmount</a>&gt; for Decimal"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"etopay_sdk/types/users/struct.UserEntity.html\" title=\"struct etopay_sdk::types::users::UserEntity\">UserEntity</a>&gt; for <a class=\"struct\" href=\"etopay_sdk/types/users/struct.ActiveUser.html\" title=\"struct etopay_sdk::types::users::ActiveUser\">ActiveUser</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.1/core/num/error/struct.ParseIntError.html\" title=\"struct core::num::error::ParseIntError\">ParseIntError</a>&gt; for <a class=\"enum\" href=\"etopay_sdk/share/enum.ShareError.html\" title=\"enum etopay_sdk::share::ShareError\">ShareError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;ApiCryptoCurrency&gt; for <a class=\"enum\" href=\"etopay_sdk/types/currencies/enum.Currency.html\" title=\"enum etopay_sdk::types::currencies::Currency\">Currency</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;DecodeError&gt; for <a class=\"enum\" href=\"etopay_sdk/share/enum.ShareError.html\" title=\"enum etopay_sdk::share::ShareError\">ShareError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Error&gt; for <a class=\"enum\" href=\"etopay_sdk/enum.WalletError.html\" title=\"enum etopay_sdk::WalletError\">WalletError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Error&gt; for <a class=\"enum\" href=\"etopay_sdk/enum.WalletError.html\" title=\"enum etopay_sdk::WalletError\">WalletError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Error&gt; for <a class=\"enum\" href=\"etopay_sdk/enum.WalletError.html\" title=\"enum etopay_sdk::WalletError\">WalletError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Error&gt; for <a class=\"enum\" href=\"etopay_sdk/enum.WalletError.html\" title=\"enum etopay_sdk::WalletError\">WalletError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Error&gt; for <a class=\"enum\" href=\"etopay_sdk/enum.WalletError.html\" title=\"enum etopay_sdk::WalletError\">WalletError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Error&gt; for <a class=\"enum\" href=\"etopay_sdk/enum.WalletError.html\" title=\"enum etopay_sdk::WalletError\">WalletError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Error&gt; for <a class=\"enum\" href=\"etopay_sdk/enum.WalletError.html\" title=\"enum etopay_sdk::WalletError\">WalletError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Error&gt; for <a class=\"enum\" href=\"etopay_sdk/error/enum.Error.html\" title=\"enum etopay_sdk::error::Error\">Error</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Error&gt; for <a class=\"enum\" href=\"etopay_sdk/kdbx/enum.KdbxStorageError.html\" title=\"enum etopay_sdk::kdbx::KdbxStorageError\">KdbxStorageError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Error&gt; for <a class=\"enum\" href=\"etopay_sdk/share/enum.ShareError.html\" title=\"enum etopay_sdk::share::ShareError\">ShareError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;FailedUnlock&gt; for <a class=\"enum\" href=\"etopay_sdk/kdbx/enum.KdbxStorageError.html\" title=\"enum etopay_sdk::kdbx::KdbxStorageError\">KdbxStorageError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;FromHexError&gt; for <a class=\"enum\" href=\"etopay_sdk/enum.WalletError.html\" title=\"enum etopay_sdk::WalletError\">WalletError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;KeyGenerationError&gt; for <a class=\"enum\" href=\"etopay_sdk/kdbx/enum.KdbxStorageError.html\" title=\"enum etopay_sdk::kdbx::KdbxStorageError\">KdbxStorageError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;KycStep&gt; for <a class=\"enum\" href=\"etopay_sdk/types/viviswap/enum.ViviswapVerificationStep.html\" title=\"enum etopay_sdk::types::viviswap::ViviswapVerificationStep\">ViviswapVerificationStep</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;KycVerificationStatus&gt; for <a class=\"enum\" href=\"etopay_sdk/types/viviswap/enum.ViviswapVerificationStatus.html\" title=\"enum etopay_sdk::types::viviswap::ViviswapVerificationStatus\">ViviswapVerificationStatus</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;LocalSignerError&gt; for <a class=\"enum\" href=\"etopay_sdk/enum.WalletError.html\" title=\"enum etopay_sdk::WalletError\">WalletError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;OpenError&gt; for <a class=\"enum\" href=\"etopay_sdk/kdbx/enum.KdbxStorageError.html\" title=\"enum etopay_sdk::kdbx::KdbxStorageError\">KdbxStorageError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;PendingTransactionError&gt; for <a class=\"enum\" href=\"etopay_sdk/enum.WalletError.html\" title=\"enum etopay_sdk::WalletError\">WalletError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;RpcError&lt;TransportErrorKind&gt;&gt; for <a class=\"enum\" href=\"etopay_sdk/enum.WalletError.html\" title=\"enum etopay_sdk::WalletError\">WalletError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Transaction&gt; for <a class=\"struct\" href=\"etopay_sdk/types/transactions/struct.WalletTxInfo.html\" title=\"struct etopay_sdk::types::transactions::WalletTxInfo\">WalletTxInfo</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;UnlockError&gt; for <a class=\"enum\" href=\"etopay_sdk/kdbx/enum.KdbxStorageError.html\" title=\"enum etopay_sdk::kdbx::KdbxStorageError\">KdbxStorageError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;WriteError&gt; for <a class=\"enum\" href=\"etopay_sdk/kdbx/enum.KdbxStorageError.html\" title=\"enum etopay_sdk::kdbx::KdbxStorageError\">KdbxStorageError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.1/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.1/std/primitive.array.html\">12</a>]&gt; for <a class=\"struct\" href=\"etopay_sdk/types/newtypes/struct.EncryptionSalt.html\" title=\"struct etopay_sdk::types::newtypes::EncryptionSalt\">EncryptionSalt</a>"]]]]);
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})()
//{"start":57,"fragment_lengths":[11621]}