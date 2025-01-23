package com.etogruppe.examples;

import com.etogruppe.CryptpaySdk;

public class GetExchangeRate14 {

    public static void main(String[] args) {

        // initialize the sdk
        CryptpaySdk sdk = utils.initSdk(utils.USERNAME_SATOSHI);

        try {

            // create and init user
            sdk.createNewUser(utils.USERNAME_SATOSHI);
            sdk.initializeUser(utils.USERNAME_SATOSHI);
            System.out.println("Created and initialized new user.");

            // get exchange rate
            double exchange_rate = sdk.getExchangeRate();
            System.out.println("Exchange rate: " + exchange_rate);

        } catch (Exception e) {
            throw new RuntimeException("Get exchange rate example failed", e);
        }
    }
}
