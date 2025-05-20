package com.etospheres.etopay.examples;

import com.etospheres.etopay.ETOPaySdk;
import com.etospheres.etopay.model.Network;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.core.JsonProcessingException;
import java.util.List;
import com.etospheres.etopay.ETOPaySdk;

public class GetExchangeRate14 {

    public static void main(String[] args) {

        // initialize the sdk
        ETOPaySdk sdk = utils.initSdk(utils.USERNAME_SATOSHI);

        try {

            // create and init user
            sdk.createNewUser(utils.USERNAME_SATOSHI);
            sdk.initializeUser(utils.USERNAME_SATOSHI);
            System.out.println("Created and initialized new user.");

            // fetch networks from backend
            sdk.getNetworks();

            // set the network configuration for the wallet
            sdk.setNetwork("iota_rebased_testnet");

            // get exchange rate
            double exchange_rate = sdk.getExchangeRate();
            System.out.println("Exchange rate: " + exchange_rate);

        } catch (Exception e) {
            throw new RuntimeException("Get exchange rate example failed", e);
        }
    }
}
