""" Music Helper
"""


class FretboardData():
    """ Fret Board load data
    """

    def __init__(self):
        self.notes = ['A', 'A#', 'B', 'C',
                      'C#', 'D', 'D#', 'E',
                      'F', 'F#', 'G', 'G#']
        self.tuning = {'std': ['E', 'A', 'B', 'G', 'D', 'E'][::-1]}
        self.scales = {'ionian': [0, 2, 4, 5, 7, 9, 11]}
        self.chords = {'maj7': [0, 4, 7, 11]}


class Fretboard():
    """ Fret Board Data Class
    """

    def __init__(self):
        self.data = FretboardData()

    def get_fretboard_strings(self, tuning: str):
        """ returns an array of strings
        """
        def note_formula(start_note, note_arr, formula):
            return [note_arr[(i + note_arr.index(start_note))
                             % len(note_arr)]
                    for i in formula]
        fretboard_string_array = [note_formula(string_note, self.data.notes,
                                               range(len(self.data.notes)))
                                  for string_note in
                                  self.data.tuning.get(tuning)]
        return fretboard_string_array

    def get_fretboard_highlights(self, key: str, formula: str):
        """ returns an array of highlight notes
        """
        key_idx = self.data.notes.index(key)
        formula_notes = self.data.scales.get(formula)
        formula_chord = self.data.chords.get(formula)

        highlights = [self.data.notes[(formula_idx + key_idx) % 12]
                      for formula_idx in (formula_scale if formula_scale != None else formula_chord)]
        return highlights


class FretboardDisplay():
    """ Fret Board Display
    """

    def __init__(self):
        self.spacing = 5
        self.fret_sep = '|'

    def print(self, fretboard, highlights):
        """ returns constructed string of highlighted
            frets
        """
        return "\n".join([self.fret_sep.join(
            [("\033[31m" if note in highlights else "\033[1m ") +
             f"{note:^5s}\033[0m" for note in fret_string])
            for fret_string in fretboard])


if __name__ == '__main__':
    fb = Fretboard()
    fd = FretboardDisplay()
    print(fd.print(fb.get_fretboard_strings(tuning='std'),
                   fb.get_fretboard_highlights(key='F#', formula='ionian')))

    print(fd.print(fb.get_fretboard_strings(tuning='std'),
                   fb.get_fretboard_highlights(key='F#', formula='maj7')))
