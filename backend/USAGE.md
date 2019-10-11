# API Usage

## Returns

The field named `status` determines the format of the response. If `status == "ok"` we have

```
{
    "status": "ok",
    "response": T,
}
```

If `status == "error"`
```
{
    "status": "error",
    "reason": Error,
}
```

## Errors

 - `"internal"` Internal error
 - `{"notFound": "thing"}` Thing was not found in the database
 - `invalidUuid` The provided uuid was not an uuid
 - `{"unknownProduct: id"` The product id did not exist

## Calls

The following give the route, the usage and the response type.
`id` is an Uuid in routes.

 - GET `/account/<id>/balance` Retrieves the balance of the user `id`, in cents:
```
{
    "balance": b
}
```
 - GET `/account/<id>/negative` Returns information on since when `id` is in negative.
```
{
    "time": "date",
    "amount_of_transactions": count,
}
```
 - GET `/account/<id>/transactions` Returns the transactions of a given account.
```
[
    {
        "time": "date-time",
        "regularization": amount,
        "products": {id: count},
        "reductions": {id: count},
    }
]
```
 - GET `/products` Gets a list of all products.
```
[
    {
        "name": "",
        "category": "",
        "price": p,
        "id": id,
    }
]
```
 - POST `/products` You need to supply, where in `[days]` `days` are of the form `Mon` or `Monday`
```
{
    "name": "",
    "category": "",
    "price": p,
    "days_active": [days],
}
```
This adds the product and returns 
```
{
    "name": "",
    "id": i,
}
```
 - POST `/account` You need to supply
```
{
    "name": "foo bar",
}
```
This creates an account and returns
```
{
    "id": "UUID",
    "name": "foo bar",
}
```

 - GET `/account/<search>` Search for a account that contains `search`. If `search == "-all-"` returns all accounts.
```
[
    {
        "id": "UUID",
        "name": "foo bar",
        "balance": b,
    }
]
```

Cards are identified with `card_id`, an integer.

 - GET `/account/<id>/cards` List all cards belonging to an account
```
[
    card_id,
]
```
 - DELETE `/account/<id>/cards/<card_id>` deletes the card from the account. No response
 - POST `/account/<id>/cards/<card_id>` adds the card to the account. No response
