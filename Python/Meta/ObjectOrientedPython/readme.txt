Prisoner's Dilemma
==================

You and your accomplice are both arrested for a crime
    - You both did commit the crime
    - 3 years of jail time
There is only evidence for conviction of a lesser crime
    - 1 year of jail time
You're both interrogated separately
    - You can:
        - Confess and rat out your accomplice, or ...
        - Not confess
    - You cannot know what your accomplice will say
The jail time you receive depends on what you both say

                                              You...
                +----------------+---------------+---------+
                | Your jail time | don't confess | confess |
                +----------------+---------------+---------+
                |  don't confess |       1       |    0    |
Accomplice...   +----------------+---------------+---------+
                |    confess     |       3       |    2    |
                +----------------+---------------+---------+

1. Make a prisoner class
2. Implement __init__(self
                          , both_confess_sentence
                          , none_confess_sentence
                          , self_confess_sentence
                          , other_confess_sentence)
3. Implement decide(self)
    return True to confess and False to keep your mouth shut
4. Implement sentence(self, years)
    so you know the outcome of your decision