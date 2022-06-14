class Prisoner:

    def __init__(
            self,
            both_confess_sentence: int,
            none_confess_sentence: int,
            self_confess_sentence: int,
            other_confess_sentence: int
    ):
        self.both_confess_sentence = both_confess_sentence
        self.none_confess_sentence = none_confess_sentence
        self.self_confess_sentence = self_confess_sentence
        self.other_confess_sentence = other_confess_sentence

        self.other_confess = False

    def decide(self) -> bool:
        if self.other_confess:
            return True
        else:
            return 2 * self.none_confess_sentence > 2 * self.both_confess_sentence or \
                   2 * self.none_confess_sentence > self.self_confess_sentence + self.other_confess_sentence

    def sentence(self, years: int) -> int:
        if not self.decide():
            self.other_confess = 2 * self.none_confess_sentence > 2 * self.both_confess_sentence or \
                   2 * self.none_confess_sentence > self.self_confess_sentence + self.other_confess_sentence

            if self.other_confess:
                years = self.other_confess_sentence
            else:
                years = self.none_confess_sentence
        else:
            years = self.both_confess_sentence
        return years




