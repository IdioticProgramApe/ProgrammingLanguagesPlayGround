from dataclasses import dataclass, field
from typing import List

from account import Account
from StateDesign.transaction import Transaction


@dataclass
class Deposit(Transaction):
    account: Account
    amount: int

    @property
    def transaction_details(self) -> str:
        # using integer to represent dollar cents
        return f"Deposited ${self.amount / 100:.2f} to account {self.account.name}"

    def execute(self) -> None:
        self.account.deposit(self.amount)


@dataclass
class Withdrawal(Transaction):
    account: Account
    amount: int

    @property
    def transaction_details(self) -> str:
        # using integer to represent dollar cents
        return f"Withdrawn ${self.amount / 100:.2f} from account {self.account.name}"

    def execute(self) -> None:
        self.account.withdraw(self.amount)


@dataclass
class Transfer(Transaction):
    from_account: Account
    to_account: Account
    amount: int

    @property
    def transaction_details(self) -> str:
        return f"Transferred ${self.amount / 100:.2f} from account {self.from_account.name} to {self.to_account.name}"

    def execute(self) -> None:
        self.from_account.withdraw(self.amount)
        self.to_account.deposit(self.amount)


@dataclass
class Batch(Transaction):
    _commands: List[Transaction] = field(default_factory=list)

    @property
    def transaction_details(self) -> str:
        return "\n".join([command.transaction_details for command in self._commands])

    def execute(self) -> None:
        for command in self._commands:
            command.execute()
