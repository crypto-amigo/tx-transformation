from random import choice, randint, shuffle

categories = ["deposit", "withdrawal", "dispute", "resolve", "chargeback"]


def main():
    print("type,client,tx,amount")
    tx_ids = list(range(10000))
    client_ids = list(range(25))
    shuffle(tx_ids)
    deposit_ids = set()
    for tx_id in tx_ids:
        category = choice(categories)
        client = choice(client_ids)
        if category in ("deposit", "withdrawal"):
            amount = randint(1, 100) / choice([2.0, 3.0, 5.0])
            print("{},{},{},{:.4f}".format(category, client, tx_id, amount))
            if category == "deposit":
                deposit_ids.add(tx_id)
        elif deposit_ids:
            deposit_tx_id = choice(list(deposit_ids))
            print("{},{},{},".format(category, client, deposit_tx_id))


if __name__ == "__main__":
    main()
