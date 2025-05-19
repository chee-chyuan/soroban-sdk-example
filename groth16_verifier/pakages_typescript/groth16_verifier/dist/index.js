import { Buffer } from "buffer";
import { Client as ContractClient, Spec as ContractSpec, } from '@stellar/stellar-sdk/contract';
export * from '@stellar/stellar-sdk';
export * as contract from '@stellar/stellar-sdk/contract';
export * as rpc from '@stellar/stellar-sdk/rpc';
if (typeof window !== 'undefined') {
    //@ts-ignore Buffer exists
    window.Buffer = window.Buffer || Buffer;
}
export const Errors = {};
export class Client extends ContractClient {
    options;
    static async deploy(
    /** Options for initalizing a Client as well as for calling a method, with extras specific to deploying. */
    options) {
        return ContractClient.deploy(null, options);
    }
    constructor(options) {
        super(new ContractSpec(["AAAAAQAAAAAAAAAAAAAAD1ZlcmlmaWNhdGlvbktleQAAAAAFAAAAAAAAAAVhbHBoYQAAAAAAB9AAAAAIRzFBZmZpbmUAAAAAAAAABGJldGEAAAfQAAAACEcyQWZmaW5lAAAAAAAAAAVkZWx0YQAAAAAAB9AAAAAIRzJBZmZpbmUAAAAAAAAABWdhbW1hAAAAAAAH0AAAAAhHMkFmZmluZQAAAAAAAAACaWMAAAAAA+oAAAfQAAAACEcxQWZmaW5l",
            "AAAAAQAAAAAAAAAAAAAABVByb29mAAAAAAAAAwAAAAAAAAABYQAAAAAAB9AAAAAIRzFBZmZpbmUAAAAAAAAAAWIAAAAAAAfQAAAACEcyQWZmaW5lAAAAAAAAAAFjAAAAAAAH0AAAAAhHMUFmZmluZQ==",
            "AAAAAAAAAAAAAAAMdmVyaWZ5X3Byb29mAAAAAwAAAAAAAAACdmsAAAAAB9AAAAAPVmVyaWZpY2F0aW9uS2V5AAAAAAAAAAAFcHJvb2YAAAAAAAfQAAAABVByb29mAAAAAAAAAAAAAAtwdWJfc2lnbmFscwAAAAPqAAAH0AAAAAJGcgAAAAAAAQAAAAE="]), options);
        this.options = options;
    }
    fromJSON = {
        verify_proof: (this.txFromJSON)
    };
}
