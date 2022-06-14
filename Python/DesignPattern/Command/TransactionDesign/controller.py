from dataclasses import dataclass, field
from typing import List

from StateDesign.transaction import Transaction


@dataclass
class BankController:

    ledger: List[Transaction] = field(default_factory=list)
    current: int = 0

    @property
    def transaction_history(self) -> str:
        return "\n".join([transaction.transaction_details for transaction in self.ledger[:self.current]])

    def register(self, transaction: Transaction):
        del self.ledger[self.current:]
        self.ledger.append(transaction)
        self.current += 1

    def undo(self) -> None:
        self.current = max(0, self.current - 1)

    def redo(self) -> None:
        self.current = min(self.current + 1, len(self.ledger))

    def compute_balances(self) -> None:
        # clear the cache before calling this function
        # otherwise the balance is stacking the previous old data
        for transaction in self.ledger[:self.current]:
            transaction.execute()
