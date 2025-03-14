"use strict";
var __extends = (this && this.__extends) || (function () {
    var extendStatics = function (d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (Object.prototype.hasOwnProperty.call(b, p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };
    return function (d, b) {
        if (typeof b !== "function" && b !== null)
            throw new TypeError("Class extends value " + String(b) + " is not a constructor or null");
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g = Object.create((typeof Iterator === "function" ? Iterator : Object).prototype);
    return g.next = verb(0), g["throw"] = verb(1), g["return"] = verb(2), typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.initSdk = initSdk;
var wasm = require("../pkg");
var dotenv = require("dotenv");
var axios_1 = require("axios");
var node_localstorage_1 = require("node-localstorage");
function initSdk(username) {
    return __awaiter(this, void 0, void 0, function () {
        var sdk, url, access_token;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    dotenv.config();
                    // setup localStorage to use a file-based mock version
                    globalThis.window = { localStorage: new node_localstorage_1.LocalStorage('./local-storage') };
                    // make sure the localStorage is clear to run each example in isolation
                    try {
                        window.localStorage.clear();
                    }
                    catch (e) {
                        console.log("Could not clear local storage: ", e);
                    }
                    console.log("Starting SDK initialization...");
                    sdk = new wasm.CryptpaySdk();
                    url = process.env.EXAMPLES_BACKEND_URL;
                    if (url == undefined) {
                        throw new Error("EXAMPLES_BACKEND_URL environment variable must be present");
                    }
                    return [4 /*yield*/, sdk.setConfig("\n    {\n        \"backend_url\": \"".concat(url, "\",\n        \"log_level\": \"info\",\n        \"auth_provider\": \"standalone\"\n    }\n    "))];
                case 1:
                    _a.sent();
                    return [4 /*yield*/, generateAccessToken(username)];
                case 2:
                    access_token = _a.sent();
                    return [4 /*yield*/, sdk.refreshAccessToken(access_token)];
                case 3:
                    _a.sent();
                    return [2 /*return*/, sdk];
            }
        });
    });
}
// Custom error class for handling token errors
var TokenError = /** @class */ (function (_super) {
    __extends(TokenError, _super);
    function TokenError(message) {
        var _this = _super.call(this, message) || this;
        _this.name = "TokenError";
        return _this;
    }
    TokenError.missingEnvironmentVariable = function (message) {
        return new TokenError("Missing environment variable: ".concat(message));
    };
    TokenError.invalidURL = function () {
        return new TokenError('Invalid URL');
    };
    TokenError.parsingError = function (message) {
        return new TokenError("Parsing error: ".concat(message));
    };
    TokenError.accessTokenNotFound = function () {
        return new TokenError('Access token not found');
    };
    return TokenError;
}(Error));
// Generate an access token by making a call to the KC API. This is mirroring the `hello.http` endpoint
function generateAccessToken(username) {
    return __awaiter(this, void 0, void 0, function () {
        var kcURL, kcRealm, clientId, clientSecret, password, urlString, env_data, response, data, error_1;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    kcURL = process.env.KC_URL;
                    kcRealm = process.env.KC_REALM;
                    clientId = process.env.KC_CLIENT_ID;
                    clientSecret = process.env.KC_CLIENT_SECRET;
                    password = process.env.PASSWORD;
                    if (!kcURL || !kcRealm || !clientId || !clientSecret || !password) {
                        throw TokenError.missingEnvironmentVariable('One or more environment variables are missing');
                    }
                    urlString = "".concat(kcURL, "/realms/").concat(kcRealm, "/protocol/openid-connect/token");
                    env_data = {
                        grant_type: 'password',
                        scope: 'profile email openid',
                        client_id: clientId,
                        client_secret: clientSecret,
                        username: username,
                        password: password
                    };
                    _a.label = 1;
                case 1:
                    _a.trys.push([1, 3, , 4]);
                    return [4 /*yield*/, axios_1.default.post(urlString, env_data, {
                            headers: { 'content-type': 'application/x-www-form-urlencoded' },
                        })];
                case 2:
                    response = _a.sent();
                    data = response.data;
                    if (data && data.access_token) {
                        return [2 /*return*/, data.access_token];
                    }
                    else {
                        throw TokenError.accessTokenNotFound();
                    }
                    return [3 /*break*/, 4];
                case 3:
                    error_1 = _a.sent();
                    if (error_1.response) {
                        // Server responded with a status other than 2xx
                        throw TokenError.parsingError("Server responded with status ".concat(error_1.response.status, ": ").concat(error_1.response.statusText));
                    }
                    else if (error_1.request) {
                        // No response was received
                        throw TokenError.invalidURL();
                    }
                    else {
                        // Something happened in setting up the request
                        throw TokenError.parsingError(error_1.message);
                    }
                    return [3 /*break*/, 4];
                case 4: return [2 /*return*/];
            }
        });
    });
}
