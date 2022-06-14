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
        return f"${self.amount / 100:.2f} to account {self.account.name}"

    def execute(self) -> None:
        self.account.deposit(self.amount)
        print(f"Deposited {self.transaction_details}")

    def undo(self) -> None:
        self.account.withdraw(self.amount)
        print(f"Undid deposit of {self.transaction_details}")

    def redo(self) -> None:
        self.account.deposit(self.amount)
        print(f"Redid deposit of {self.transaction_details}")


@dataclass
class Withdrawal(Transaction):
    account: Account
    amount: int

    @property
    def transaction_details(self) -> str:
        # using integer to represent dollar cents
        return f"${self.amount / 100:.2f} from account {self.account.name}"

    def execute(self) -> None:
        self.account.withdraw(self.amount)
        print(f"Withdrawn {self.transaction_details}")

    def undo(self) -> None:
        self.account.deposit(self.amount)
        print(f"Undid withdrawal of {self.transaction_details}")

    def redo(self) -> None:
        self.account.withdraw(self.amount)
        print(f"Redid withdrawal of {self.transaction_details}")


@dataclass
class Transfer(Transaction):
    from_account: Account
    to_account: Account
    amount: int

    @property
    def transaction_details(self) -> str:
        return f"${self.amount / 100:.2f} from account {self.from_account.name} to {self.to_account.name}"

    def execute(self) -> None:
        self.from_account.withdraw(self.amount)
        self.to_account.deposit(self.amount)
        print(f"Transferred {self.transaction_details}")

    def undo(self) -> None:
        self.from_account.deposit(self.amount)
        self.to_account.withdraw(self.amount)
        print(f"Undid transaction of {self.transaction_details}")

    def redo(self) -> None:
        self.from_account.withdraw(self.amount)
        self.to_account.deposit(self.amount)
        print(f"Redid transaction of {self.transaction_details}")


@dataclass
class Batch(Transaction):
    _commands: List[Transaction] = field(default_factory=list)

    def execute(self) -> None:
        completed_commands: List[Transaction] = []
        try:
            for command in self._commands:
                command.execute()
                completed_commands.append(command)
        except ValueError as err:
            print(f"Command error {err}, reverting...")
            for command in reversed(completed_commands):
                command.undo()

    def undo(self) -> None:
        for command in reversed(self._commands):
            command.undo()

    def redo(self) -> None:
        for command in self._commands:
            command.redo()
