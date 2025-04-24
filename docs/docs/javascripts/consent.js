var _paq = window._paq = window._paq || [];
var consent = __md_get("__consent")
if (consent && consent.analytics === true) {
    /* The user accepted the cookie */
    _paq.push(['rememberConsentGiven']);
} else {
    /* The user rejected the cookie */
    _paq.push(['forgetConsentGiven']);
}