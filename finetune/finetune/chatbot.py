from model import ChatModel

class ChatBot:
    def __init__(
        self, 
        model: ChatModel,
        context_window: int = 6
    ):
        self.messages = []
        self.model = model
        self.context_window = context_window

    def add_input_and_make_prompt(self, _input: str):
        prompt_parts = []
        self.messages.append(("[OTHER]", _input));
        context = self.messages[-self.context_window:]
        for (author_label, content) in context:
            prompt_parts.append(f"{author_label}: {content}")
        prompt_parts.append(f"[ASSISTANT]: ")
        return "\n".join(prompt_parts)

    # replies to the added message
    def add_message(self, content: str):
        prompt = self.add_input_and_make_prompt(content)
        # print("PROMPT START -----------")
        # print(prompt)
        # print("PROMPT END -----------")

        text = self.model.gen_text(prompt, temperature=0.8, max_length=80)[0]
        self.messages.append(("[ASSISTANT]", text))
        return text
        
