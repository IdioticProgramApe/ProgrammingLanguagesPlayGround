import random
import string
from dataclasses import dataclass, field
from typing import Dict

from account import Account


@dataclass
class Bank:
    accounts: Dict[str, Account] = field(default_factory=dict)

    def create_account(self, name: str) -> Account:
        number = "".join(random.choices(string.digits, k=12))
        account = Account(name, number)
        self.accounts[number] = account
        return account

    def get_account(self, account_number: str) -> Account:
        return self.accounts[account_number]

    def clear_cache(self) -> None:
        for account in self.accounts.values():
            account.clear_cache()
