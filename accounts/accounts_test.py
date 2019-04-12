#! /usr/bin/python3
from link import DataBase

db = DataBase('test_database.db')
# card_id = 0x4264e52115b800d0a
card_id = "4264e52115b800d0a"
db.add_account('aurelien.moisson-franckhauser@grenoble-inp.org','aurelien', 'moisson')

aurelien_id = db.find_id('aurelien.moisson-franckhauser@grenoble-inp.org')

acc = db.find_account_like('%', 'aure%', '%')
print(acc)
assert (aurelien_id, 'aurelien.moisson-franckhauser@grenoble-inp.org','aurelien', 'moisson') in acc

acc = db.find_account_like('%', '%', 'moiss%')
print(acc)
assert (aurelien_id, 'aurelien.moisson-franckhauser@grenoble-inp.org','aurelien', 'moisson') in acc

db.add_card(card_id, aurelien_id)

assert aurelien_id == db.get_account_from_card(card_id)

acc = db.find_account(aurelien_id)
print(acc)
assert acc == (aurelien_id, 'aurelien.moisson-franckhauser@grenoble-inp.org','aurelien', 'moisson')
