class Clock:

    def __init__(self, hour, minute):
        self.minutes = ((hour * 60) + minute) % (60 * 24)

    def __repr__(self):
        hour = self.minutes // 60
        minute = self.minutes % 60
        return f"{hour:02d}:{minute:02d}"

    def __eq__(self, other):
        return other.minutes == self.minutes

    def __add__(self, minutes):
        return Clock(0, self.minutes + minutes)

    def __sub__(self, minutes):
        return Clock(0, self.minutes - minutes)
