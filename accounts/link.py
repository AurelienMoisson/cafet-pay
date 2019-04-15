import sqlite3
from time import time
"""
library used to create records of cards and transactions
positive transactions means money added to an account
negative transactions means money removed from an account (or spent)
"""
# TODO : make sure nothing is lost if the program crashes
#        or if the computer loses power


def create_table(cursor, table_name, fields):
    table_description = ', '.join(' '.join(t) for t in fields)
    command = "CREATE TABLE {}({})".format(table_name,
                                           table_description)
    try:
        cursor.execute(command)
    except sqlite3.OperationalError:
        pass
    else:
        print(table_name + " table did not exist yet and was just created")

def insert_into(cursor, table_name, columns, values):
    col = ", ".join(columns)
    val = ", ".join('?' for v in values)
    command = "INSERT INTO {} ({}) VALUES ({})".format(table_name,
                                                       col,
                                                       val)
    cursor.execute(command, values)


class DataBase:
    def __init__(self, path):
        self.conn = sqlite3.connect(path)
        self.cursor = self.conn.cursor()
        
        #if the tables don't exist yet, create them
        create_table(self.cursor,
                     "accounts",
                     [("account_id", "integer", "PRIMARY KEY"),
                      ("email", "text"),
                      ("firstname", "text"),
                      ("lastname", "text"),
                      ("CONSTRAINT", "email_unique", "UNIQUE", "(email)"),
                      ("CONSTRAINT", "name_unique", "UNIQUE", "(firstname, lastname)")])

        create_table(self.cursor,
                     "cards",
                     [("card_id", "text", "PRIMARY KEY"),
                      ("account_id", "integer")])

        create_table(self.cursor,
                     "transactions",
                     [("transaction_id", "integer", "PRIMARY KEY"),
                      ("time", "integer"),
                      ("account_id", "integer"),
                      ("amount", "integer")])

        create_table(self.cursor,
                     "purchases",
                     [("transaction_id", "integer"),
                      ("item", "text"),
                      ("price", "integer"),
                      ("quantity", "integer")])
    def add_account(self, email, firstname, lastname):
        insert_into(self.cursor,
                    "accounts",
                    ["email", "firstname", "lastname"],
                    [email, firstname, lastname])
    def find_id(self, email):
        command = 'SELECT account_id FROM accounts WHERE email = ?'
        result = self.cursor.execute(command, [email])
        return result.fetchone()[0]
    def find_account_like(self, email='%', firstname='%', lastname='%'):
        command = 'SELECT * FROM accounts WHERE email LIKE ? AND firstname LIKE ? AND lastname LIKE ?'
        result = self.cursor.execute(command, [email, firstname, lastname])
        return list(result)
    def add_card(self, card_id, account_id):
        insert_into(self.cursor,
                "cards",
                ["card_id", "account_id"],
                [card_id, account_id])
    def get_account_from_card(self, card_id):
        command = "SELECT account_id FROM cards WHERE card_id = ?"
        result = self.cursor.execute(command, [card_id])
        return result.fetchone()[0]
    def find_account(self, account_id):
        command = "SELECT * FROM accounts WHERE account_id = ?"
        result = self.cursor.execute(command, [account_id])
        return result.fetchone()

    def add_transaction(self, account_id, items, transaction_time = None):
        """
        items : [(item name, item price, quantity bought), ...]
        transaction_time as unix time
        """
        if transaction_time == None:
            transaction_time = time()
        total = sum(t[1]*t[2] for t in items)
        # add transaction
        insert_into(self.cursor,
                    "transactions",
                    ["time", "account_id", "amount"],
                    [str(int(transaction_time)), account_id, str(total)])
        # find transaction id
        command = "SELECT transaction_id FROM transactions WHERE rowid = (SELECT last_insert_rowid())"
        transaction_id, = self.cursor.execute(command).fetchone()
        # add each item purchased
        for item_name, item_price, quantity in items:
            insert_into(self.cursor,
                        "purchases",
                        ["transaction_id", "item", "price", "quantity"],
                        [transaction_id, item_name, item_price, quantity])
