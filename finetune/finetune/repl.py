from model import ChatModel, ModelType
from dataset import ChatDataset, Turn
from chatbot import ChatBot
import json
from bot import run_bot

with open("datasets/2424_646479549550559272.json", "r") as f:
    data = json.load(f)

messages = [Turn(msg) for msg in data[0]]

batch_size = 6
learning_rate = 5e-5
output_dir = "finetuned_1"

try:
    chat_model = ChatModel.from_pretrained(output_dir, "cuda")
except:
    print("Loading default gpt-2 model")
    chat_model = ChatModel(ModelType.GPT2_MEDIUM)

chatbot = ChatBot(chat_model)

dataset = ChatDataset(messages, "qarri", chat_model.tokenizer)

while True:
    prompt = input("> ")
    if prompt.startswith("train "):
        epochs = int(prompt.split()[1])
        chat_model.finetune(
            dataset=dataset,
            epochs=epochs,
            batch_size=batch_size,
            learning_rate=learning_rate,
        )
        chat_model.save(output_dir)
        print(f"SAVED TO: {output_dir}")
    elif prompt.startswith("bot"):
        run_bot(chatbot)
    else:
        response = chatbot.add_message(prompt)
        print(response)
