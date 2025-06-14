from chatbot import ChatBot
import discord
from dotenv import load_dotenv
import os

load_dotenv()

def run_bot(chatbot: ChatBot):
    intents = discord.Intents.default()
    intents.message_content = True

    client = discord.Client(intents=intents)

    @client.event
    async def on_ready():
        await client.change_presence(status=discord.Status.dnd)
        print(f"Ready: {client.user}")

    @client.event
    async def on_message(message):
        if message.author == client.user:
            return

        if message.channel.id == 1329682105281548372 or message.channel.id == 1075946648108269702:
            messages = chatbot.add_message(message.content)
            await message.channel.send(messages)
            return
            messages = messages.splitlines()
            for msg in messages:
                if msg:
                    await message.channel.send(msg)

    client.run(os.getenv("BOT_TOKEN"))