from link import DataBase

db = DataBase('test_database.db')
card_id = 0x4264e52115b800d0a
db.add_account('aurelien', 'moisson')

aurelien_id = db.find_id('aurelien', 'moisson')

acc = db.find_account_like('aure*', '*')
assert acc[aurelien_id] == ('aurelien', 'moisson')

acc = db.find_account_like('*', 'moiss*')
assert acc[aurelien_id] == ('aurelien', 'moisson')

db.add_card(card_id, aurelien_id)

assert aurelien_id == db.get_account_from_card(card_id)

assert db.find_account(aurelien_id) == ('aurelien', 'moisson')
