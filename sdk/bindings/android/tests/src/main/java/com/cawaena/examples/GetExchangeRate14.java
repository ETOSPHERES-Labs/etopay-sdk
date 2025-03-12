package com.cawaena.examples;

import com.cawaena.Wallet;

public class GetExchangeRate14 {

    public static void main(String[] args) {

        // initialize the sdk
        Wallet sdk = utils.initSdk(utils.USERNAME_SATOSHI);

        try {

            // create and init user
            sdk.createNewUser(utils.USERNAME_SATOSHI);
            sdk.initializeUser(utils.USERNAME_SATOSHI);
            System.out.println("Created and initialized new user.");
            
            // fetch networks from backend
            sdk.getNetworks();
            // set the network configuration for the wallet
            sdk.setNetwork(utils.IOTA_NETWORK_ID);

            // get exchange rate
            double exchange_rate = sdk.getExchangeRate();
            System.out.println("Exchange rate: " + exchange_rate);

        } catch (Exception e) {
            throw new RuntimeException("Get exchange rate example failed", e);
        }
    }
}
