package com.etogruppe.examples;

import com.etogruppe.CryptpaySdk;

public class CreateNewUser01 {

    public static void main(String[] args) {

        // Initialize SDK
        CryptpaySdk sdk = utils.initSdk(utils.USERNAME_SATOSHI);

        try {
            sdk.createNewUser(utils.USERNAME_SATOSHI);
            sdk.initializeUser(utils.USERNAME_SATOSHI);
            System.out.println("Created and initialized new user.");
        } catch (Exception e) {
            throw new RuntimeException("Create new user example failed", e);
        }
    }
}
