Introduction: https://refactoring.guru/design-patterns/command
Thinking: states or transactions, which would be better for the application ?

State design:
- based on the final state and cache the transactions
- can get the result very fast but might be difficult to get one specific transaction

Transaction design:
- based on the transactions and cache the final state
- can list in detail all the transactions in history but need to build the result from the start

Consider the application actual usage to decide which one to use!
