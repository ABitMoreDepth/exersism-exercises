"""
    clock that handles times without dates.
"""


class Clock():
    def __init__(self, hours, minutes):
        if (hours is not None
                and minutes is not None
                and isinstance(hours, int)
                and isinstance(minutes, int)):
            self.hours = hours
            self.minutes = minutes
        else:
            raise ValueError('Insufficient Arguments provided')

        self._process_inputs()

    def _process_inputs(self):
        """
            Parses class arguments to ensure they are standard readable time
            values, translates minus numbers and numbers out of range etc.
        """
        hours_from_minutes = self.minutes // 60  # 60 minutes in an hours
        minutes_left = self.minutes % 60  # Grab the leftover

        # Adjust hours and minute setting accordingly
        self.minutes = minutes_left
        self.hours += hours_from_minutes

        # Wrap hours to max. 24
        self.hours = self.hours % 24

    def __repr__(self):
        """
            Called by the repr() built-in function and by string converions.

            This is triggered by wrapping a class instantiation inside a str()
            call.
        """
        return '{hours:02d}:{minutes:02d}'.format(
            hours=self.hours,
            minutes=self.minutes)

    def __eq__(self, other):
        """
            Called by the == operator on a class instance.
        """
        return self.minutes == other.minutes and self.hours == other.hours

    def add(self, minutes):
        """
            Adds time to the clock, supports negative time
        """
        if isinstance(minutes, int):
            self.minutes += minutes
            self._process_inputs()

            return self.__repr__()
