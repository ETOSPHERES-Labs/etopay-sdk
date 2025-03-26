# Dashboard

The backend to the ETOPay SDK is accessible through our [dashboard](https://etopayapp.etospheres.com) where you can:

1. Login using GitHub.
2. Manage your active projects.
3. Retrieve the [configuration](../SDK%20Configuration/Configuration.md) needed to use the ETOPay SDK.
3. Configure secrets required for integrating with [KYC providers](../SDK%20Modules/KYC%20Onboarding.md).

## Login using GitHub

Open the [dashboard](https://etopayapp.etospheres.com) and press the button to login through your [GitHub](https://github.com) account.
By logging in an using our platform you agree to the [Terms of Service](https://etopayapp.etospheres.com/terms_of_service) and [Privacy Policy](https://etopayapp.etospheres.com/privacy_policy).
When authorizing the GitHub application, you also agree to that we can access the following information about your account:

- Your primary email address (even if it is marked as _private_). We use this information to communicate important updates and required actions about your projects.

## Manage Projects


### Creating a Project

After logging in for the first time, you'll be greated with a page that allows you to create a new project by signing up for your preferred plan.

???+ info
    The payment is entirely handled through [Stripe](https://stripe.com). We do not store or have access you your payment details at any time.
    After paying and creating a project you can manage your subscription at any time through the _Manage Subscription_ navigation link.

When signing up for a new plan a new project is created in your list of projects. You can create a new project, with your preferred plan, using the _Create New Project_ button.

Each project includes two instances of the ETOPay services:

1. A `test` instance used for testing during integration and development.
2. A `live` instance used for the actual production usage.

After creating a project, follow the _Setup Guide_ shown on the _Project Details_ page to perform the required one-time setup of the Keycloak administrator accounts that you will use to manage your end-users.

### Managing a Project

Selecting a project brings you to the _Project Details_ page. Using the toggle in the lower left corner you can select between the `test` and the `live` instance.
For each instance, you can see statistics about the number of Montly Active Wallets (MAWs) and the number of purchases that have been performed on your instance.
Using the navigation on the left side you can also visit the following pages:

- **Project Details**: The main page with statistics and information about your project, including the _Setup Guide_.
- **User Management**: Once your instance is created, you can use this link to manage your users in Keycloak.
- **Purchases**: Coming soon: Here you can inspect the purchases that were performed through your instance.
- **Manage Subscription**: This link will take you to _Stripe_ to manage your subscription details.
- **Project Settings**: Here you can manage settings about your project instance. This has three tabs:
    - **General**: General settings like project name and description. This is also where you can archive your project.
    - **Wallet**: Here you see the list of pre-configured networks we support. In the future you will be able to add your own here.
    - **Secrets**: This is where you [configure secrets](#configure-secrets) for integrating with [KYC providers](../SDK%20Modules/KYC%20Onboarding.md).


## Retrieving Configuration

To retrieve the configuration needed for the SDK, go to the _Project Details_ page of your project.
Under _Setup Guide_ you will find a template configuration JSON that you can use to initialize the SDK.
See also [Configuration](../SDK%20Configuration/Configuration.md) for details on configuring the SDK.

## Configuring Secrets

To use any of the supported [KYC providers](../SDK%20Modules/KYC%20Onboarding.md) for enabling Know Your Customer (KYC) on your instance, visit the _Secrets_ tab under _Project Settings_.
On this page, you can enter your access credentials for respective provider. Note that you need to do this for the `live` and `test` instace separately.
If you need to rotate the credentials you can update them at any time, but you need to provide all values every time you save.

???+ info
    The secret values you enter are write-only. Thus there is no way to retrieve the secret values once they are saved.
    Hence the text inputs will stay empty when the page is refreshed even when values are stored.
