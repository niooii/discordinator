from typing import List, Dict
from torch.utils.data import Dataset, DataLoader
from transformers import GPT2LMHeadModel, GPT2Tokenizer, AdamW, get_linear_schedule_with_warmup

class Turn:
    def __init__(self, message: Dict):
        self.author = message["author"]
        self.content = message["content"]
        self.timestamp = message["start_time"]

    def __str__(self):
       return f"Turn({self.author} - {self.content})"

    def __repr__(self):
       return f"Turn({self.author} - {self.content})"


class ChatDataset(Dataset):
    def __init__(self, messages: List[Turn], target_author: str, tokenizer: GPT2Tokenizer, max_length: int = 128):
        self.messages = self._filter_and_process_messages(messages, target_author)
        self.tokenizer = tokenizer
        self.max_length = max_length
        
    def _filter_and_process_messages(self, messages: List[Turn], target_author: str) -> List[str]:
        filtered_messages = [msg.content for msg in messages if msg.author.lower() == target_author.lower()]
        
        # no significant processing for now
        processed_messages = []
        for message in filtered_messages:
            if message:
                processed_messages.append(message)
            
        return processed_messages
    
    def __len__(self):
        return len(self.messages)
    
    def __getitem__(self, idx):
        message = self.messages[idx]
        
        encoding = self.tokenizer(
            message,
            add_special_tokens=True,
            max_length=self.max_length,
            padding="max_length",
            truncation=True,
            return_tensors="pt"
        )
        
        # Create input_ids and attention_mask
        input_ids = encoding["input_ids"].squeeze()
        attention_mask = encoding["attention_mask"].squeeze()
        
        # For GPT-2 training, labels are the same as inputs
        labels = input_ids.clone()
        
        # Mask out padding tokens in labels
        labels[attention_mask == 0] = -100
        
        return {
            "input_ids": input_ids,
            "attention_mask": attention_mask,
            "labels": labels
        }