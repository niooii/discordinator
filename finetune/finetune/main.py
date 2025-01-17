import torch
from datasets import Dataset
import json
from dataset import ChatDataset, Turn
from model import ChatModel, ModelType
from transformers import GPT2LMHeadModel, GPT2Tokenizer, AdamW, get_linear_schedule_with_warmup
from torch.utils.data import Dataset, DataLoader

with open("datasets/2424_646479549550559272.json", "r") as f:
    data = json.load(f)

messages = [Turn(msg) for msg in data[0]]
# print(messages)

num_epochs = 10
batch_size = 4
learning_rate = 5e-5
max_length = 128
output_dir = "finetuned_1"

chat_model = ChatModel(ModelType.GPT2_MEDIUM)
# chat_model = ChatModel.from_pretrained("finetuned_1", "cuda")

dataset = ChatDataset(messages, "qarri", chat_model.tokenizer)

chat_model.finetune(
    dataset=dataset,
    epochs=num_epochs,
    batch_size=batch_size,
    learning_rate=learning_rate,
    save_steps=1,
    output_dir=output_dir
)

res = chat_model.gen_text("WHERE R")
print(f"Got {len(res)} responses:")
for r in res:
    print(r)

chat_model.save(output_dir)
