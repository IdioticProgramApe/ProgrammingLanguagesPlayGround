import sys

from bank import Bank


def state_main() -> None:
    # local imports
    from StateDesign.controller import BankController
    from StateDesign.commands import Deposit, Withdrawal, Transfer, Batch

    # create a bank
    bank = Bank()

    # create a bank controller
    controller = BankController()

    # create some accounts
    account1 = bank.create_account('Alpha')
    account2 = bank.create_account('Beta')
    account3 = bank.create_account('Gamma')

    controller.execute(Batch([Deposit(account1, 100000), Deposit(account2, 100000), Deposit(account3, 100000)]))
    print(bank)     # keypoint1

    # transfer
    controller.execute(Transfer(account2, account1, 50000))
    controller.execute(Withdrawal(account1, 150000))
    print(bank)     # keypoint2

    # undo
    controller.undo()
    controller.undo()
    print(bank)     # should be like keypoint1

    # redo
    controller.redo()
    controller.redo()
    print(bank)     # should be like keypoint2


def transaction_main() -> None:
    # local imports
    from TransactionDesign.controller import BankController
    from TransactionDesign.commands import Deposit, Withdrawal, Transfer, Batch

    # create a bank
    bank = Bank()

    # create a bank controller
    controller = BankController()

    # create some accounts
    account1 = bank.create_account('Alpha')
    account2 = bank.create_account('Beta')
    account3 = bank.create_account('Gamma')

    controller.register(Batch([Deposit(account1, 100000), Deposit(account2, 100000), Deposit(account3, 100000)]))
    bank.clear_cache()
    print(controller.transaction_history)
    controller.compute_balances()
    print(bank)     # keypoint1

    # transfer
    controller.register(Transfer(account2, account1, 50000))
    controller.register(Withdrawal(account1, 150000))
    bank.clear_cache()
    print(controller.transaction_history)
    controller.compute_balances()
    print(bank)     # keypoint2

    # undo
    controller.undo()
    controller.undo()
    bank.clear_cache()
    print(controller.transaction_history)
    controller.compute_balances()
    print(bank)     # should be like keypoint1

    # redo
    controller.redo()
    controller.redo()
    bank.clear_cache()
    print(controller.transaction_history)
    controller.compute_balances()
    print(bank)     # should be like keypoint2


if __name__ == "__main__":
    if sys.argv[1] == "state":
        state_main()
    elif sys.argv[1] == 'transaction':
        transaction_main()
    else:
        print("Nothing is triggered!")
