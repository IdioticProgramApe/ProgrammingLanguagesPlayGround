from dataclasses import dataclass, field
from typing import List

from StateDesign.transaction import Transaction


@dataclass
class BankController:

    undo_stack: List[Transaction] = field(default_factory=list)
    redo_stack: List[Transaction] = field(default_factory=list)

    def execute(self, transaction: Transaction):
        transaction.execute()
        self.undo_stack.append(transaction)

    def undo(self) -> None:
        if not self.undo_stack:
            return
        transaction = self.undo_stack.pop()
        transaction.undo()
        self.redo_stack.append(transaction)

    def redo(self) -> None:
        if not self.redo_stack:
            return
        transaction = self.redo_stack.pop()
        transaction.redo()
        self.undo_stack.append(transaction)
