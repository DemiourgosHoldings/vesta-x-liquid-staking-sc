import {
    ArgSerializer,
    Balance,
    BigUIntValue,
    BytesValue,
    guardValueIsSet,
    TransactionPayload,
    TypedValue
} from "@elrondnetwork/erdjs";
import BigNumber from 'bignumber.js';
import * as fs from 'fs';

export const sleep = (ms) => new Promise(resolve => setTimeout(resolve, ms));

export const convertWeiToEsdt = (amount: string | number | BigNumber | null, decimals?: number, precision?: number): BigNumber => {
    if (amount == null) {
        return BigNumber(0);
    } else {
        return BigNumber(amount).shiftedBy(typeof(decimals) !== 'undefined' ? -decimals : -18).decimalPlaces(typeof(precision) !== 'undefined' ? precision : 4, BigNumber.ROUND_DOWN);
    }
};

export const convertEsdtToWei = (amount: string | number | BigNumber, decimals?: number): BigNumber => {
    return BigNumber(amount).shiftedBy(typeof(decimals) !== 'undefined' ? decimals : 18);
};

export const readLinesFromFile = (filePath: string) => {
    const whitelist = fs.readFileSync(filePath, 'utf-8').trim().replace(/(\r\n|\n|\r)/gm, '').split(",");
    return whitelist;
}