import os
from typing import List, Dict, Any

import requests
from bs4 import BeautifulSoup
from telegram import InlineKeyboardButton, InlineKeyboardMarkup
from telegram.ext import Updater, CommandHandler, CallbackQueryHandler


TOKEN = os.getenv('BOT_TOKEN')
URL = os.getenv('BOT_URL', default='https://www.studentenwerk-dresden.de/mensen/speiseplan/?view=list')

print(f"Token: {TOKEN}")
print(f"Url: {URL}")


def get_canteen_names() -> List[str]:
    return list(get_all_canteens().keys())


def get_all_canteens() -> Dict[str, Any]:
    content_raw = requests.get(URL).content
    content = BeautifulSoup(content_raw, 'html.parser')
    cards = content.select('.card')

    found = {}
    for card in cards:
        headers = card.select('h3.card-header')
        if not headers:
            continue

        header = headers[0].text
        found[header] = card

    return found


def get_dishes_for_card(card):
    menu_items = card.select('li.swdd-link-list-item')
    dishes = list()
    for index, menu_item in enumerate(menu_items):

        dish_name = menu_item.select('span')[1].text
        dish_price = menu_item.select('strong')[0].text
        print(f'Dish {index}')
        print(f'- {dish_name}')
        if dish_price:
            dish_price = dish_price.split('/')[0].strip().replace(' ', '')
            print(f'=> {dish_price}')

        dishes.append({
            'name': dish_name,
            'price': dish_price
        })

    return dishes


def get_lunch(name: str) -> list:
    canteens = get_all_canteens()
    return get_dishes_for_card(canteens[name])


def list_action(update, context):
    names = get_canteen_names()
    keyboard = [
        [InlineKeyboardButton(name, callback_data=name)]
        for name in names
    ]
    reply_markup = InlineKeyboardMarkup(keyboard)
    update.message.reply_text('🤔 Choose a mensa ⬇', reply_markup=reply_markup)


def list_callback(update, context):
    query = update.callback_query
    selected = query.data

    dishes = get_lunch(selected)

    if not dishes:
        query.edit_message_text(text='No dishes currently')

    query.edit_message_text(text=f'Dishes for {selected}')

    for dish in dishes:
        query.message.reply_text(f"{dish['name']} {dish['price']}")


def start_bot():
    updater = Updater(TOKEN, use_context=True)

    updater.dispatcher.add_handler(CommandHandler('list', list_action))
    updater.dispatcher.add_handler(CallbackQueryHandler(list_callback))

    updater.start_polling()
    updater.idle()


start_bot()
