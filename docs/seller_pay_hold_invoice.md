# Seller pays hold invoice

## Overview

All Mostro messages are [Parameterized Replaceable Events](https://github.com/nostr-protocol/nips/blob/master/01.md#kinds) and use `30078` as event `kind`, a list of standard event kinds can be found [here](https://github.com/nostr-protocol/nips#event-kinds)

## Communication between users and Mostro

All messages from/to Mostro should be a Nostr event [kind 4](https://github.com/nostr-protocol/nips/blob/master/04.md), the `content` field of the event should be a base64-encoded, aes-256-cbc encrypted JSON-serialized string (with no white space or line breaks) of the following structure:

- `version`
- `order_id` (optional)
- `pubkey` (optional)
- `action` (https://docs.rs/mostro-core/latest/mostro_core/enum.Action.html)
- `content` (optional https://docs.rs/mostro-core/latest/mostro_core/enum.Content.html)

## Paying a hold invoice

When the seller is the maker and the order was taken by a buyer, Mostro will send to the seller a message asking to pay the hold invoice, the message will look like this:

```json
{
  "version": "0",
  "order_id": "ede61c96-4c13-4519-bf3a-dcf7f1e9d842",
  "pubkey": null,
  "action": "PayInvoice",
  "content": {
    "PaymentRequest": [
      {
        "id": "ede61c96-4c13-4519-bf3a-dcf7f1e9d842",
        "kind": "Sell",
        "status": "WaitingBuyerInvoice",
        "amount": 7851,
        "fiat_code": "VES",
        "fiat_amount": 100,
        "payment_method": "face to face",
        "premium": 1,
        "created_at": 1698937797
      },
      "lnbcrt78510n1pj59wmepp50677g8tffdqa2p8882y0x6newny5vtz0hjuyngdwv226nanv4uzsdqqcqzzsxqyz5vqsp5skn973360gp4yhlpmefwvul5hs58lkkl3u3ujvt57elmp4zugp4q9qyyssqw4nzlr72w28k4waycf27qvgzc9sp79sqlw83j56txltz4va44j7jda23ydcujj9y5k6k0rn5ms84w8wmcmcyk5g3mhpqepf7envhdccp72nz6e"
    ]
  }
}
```

After the hold invoice is paid Mostro will send a new message to seller with the following content:

```json
{
  "version": "0",
  "order_id": "ede61c96-4c13-4519-bf3a-dcf7f1e9d842",
  "pubkey": null,
  "action": "BuyerTookOrder",
  "content": {
    "SmallOrder": {
      "id": "ede61c96-4c13-4519-bf3a-dcf7f1e9d842",
      "amount": 7851,
      "fiat_code": "VES",
      "fiat_amount": 100,
      "payment_method": "face to face",
      "premium": 1,
      "buyer_pubkey": "npub1qqqt938cer4dvlslg04zwwf66ts8r3txp6mv79cx2498pyuqx8uq0c7qkj",
      "seller_pubkey": "npub1qqqxssz4k6swex94zdg5s4pqx3uqlhwsc2vdzvhjvzk33pcypkhqe9aeq2"
    }
  }
}
```

Mostro also send a message to the buyer, this way they can both to to each other in private, this message would look like this:

```json
{
  "version": "0",
  "order_id": "ede61c96-4c13-4519-bf3a-dcf7f1e9d842",
  "pubkey": null,
  "action": "HoldInvoicePaymentAccepted",
  "content": {
    "SmallOrder": {
      "id": "ede61c96-4c13-4519-bf3a-dcf7f1e9d842",
      "amount": 7851,
      "fiat_code": "VES",
      "fiat_amount": 100,
      "payment_method": "face to face",
      "premium": 1,
      "buyer_pubkey": "npub1qqqt938cer4dvlslg04zwwf66ts8r3txp6mv79cx2498pyuqx8uq0c7qkj",
      "seller_pubkey": "npub1qqqxssz4k6swex94zdg5s4pqx3uqlhwsc2vdzvhjvzk33pcypkhqe9aeq2"
    }
  }
}
```

## Ephemeral keys

Mostro clients should use ephemeral keys to communicate with Mostro, indicating the pubkey where they want to be contacted in the `pubkey` field of the message, this way orders and users can't be easily linked, `buyer_pubkey` and `seller_pubkey` fields are the real pubkeys of the users.
