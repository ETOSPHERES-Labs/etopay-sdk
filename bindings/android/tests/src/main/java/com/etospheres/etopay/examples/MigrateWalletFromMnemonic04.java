package com.etospheres.etopay.examples;

import com.etospheres.etopay.ETOPaySdk;
import com.etospheres.etopay.model.Network;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.core.JsonProcessingException;
import java.util.List;
import com.etospheres.etopay.ETOPaySdk;

public class MigrateWalletFromMnemonic04 {

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
            System.out.println("Created new wallet from mnemonic.");

            // fetch networks from backend
            String networks = sdk.getNetworks();

            List<Network> networksList;
            try {
                ObjectMapper objectMapper = new ObjectMapper();
                networksList = objectMapper.readValue(networks, new TypeReference<List<Network>>() {
                });
            } catch (JsonProcessingException e) {
                throw new RuntimeException("Error processing JSON response", e);
            }

            Network iotaNetwork = networksList.get(0);
            // set the network configuration for the wallet
            sdk.setNetwork(iotaNetwork.key);

        } catch (Exception e) {
            throw new RuntimeException("Migrate wallet from mnemonic example failed", e);
        }
    }
}
