package com.etospheres.etopay.examples;

import com.etospheres.etopay.ETOPaySdk;
import com.etospheres.etopay.model.Network;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.core.JsonProcessingException;
import java.util.List;
import com.etospheres.etopay.ETOPaySdk;

public class GenerateNewAddress06 {

    public static void main(String[] args) {

        // initialize the sdk
        ETOPaySdk sdk = utils.initSdk(utils.USERNAME_SATOSHI);

        String password = "correcthorsebatterystaple";
        String mnemonic = utils.getEnvVariable("MNEMONIC");

        try {
            // create and init user
            sdk.createNewUser(utils.USERNAME_SATOSHI);
            sdk.initializeUser(utils.USERNAME_SATOSHI);
            System.out.println("Created and initialized new user.");

            // create new wallet
            sdk.setWalletPassword(utils.PIN, password);
            sdk.createWalletFromMnemonic(utils.PIN, mnemonic);
            System.out.println("Created and initialized new wallet from mnemonic.");

            // fetch networks from backend
            sdk.getNetworks();

            // set the network configuration for the wallet
            sdk.setNetwork("iota_rebased_testnet");

            // generate receiver address
            String address = sdk.generateNewAddress(utils.PIN);
            System.out.println("First Address: " + address);

            // update the account and index to get another address
            sdk.setWalletAccount(0, 1);
            String address2 = sdk.generateNewAddress(utils.PIN);
            System.out.println("Second Address: " + address2);

        } catch (Exception e) {
            throw new RuntimeException("Generate new address example failed", e);
        }
    }
}
