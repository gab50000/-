from dataclasses import dataclass
import pathlib
import re


@dataclass
class Passport:
    byr: str | None = None
    iyr: str | None = None
    eyr: str | None = None
    hgt: str | None = None
    hcl: str | None = None
    ecl: str | None = None
    pid: str | None = None
    cid: str | None = None

    def is_valid(self):
        return all(
            [
                self.byr,
                self.iyr,
                self.eyr,
                self.hgt,
                self.hcl,
                self.ecl,
                self.pid,
            ]
        )

    def is_valid2(self):
        try:
            return all(
                [
                    self.byr and 1920 <= int(self.byr) <= 2002,
                    self.iyr and 2010 <= int(self.iyr) <= 2020,
                    self.eyr and 2020 <= int(self.eyr) <= 2030,
                    self.hgt
                    and (re.match(r"^\d+(cm|in)$", self.hgt))
                    and (
                        (150 <= int(self.hgt[:-2]) <= 193)
                        if self.hgt.endswith("cm")
                        else (59 <= int(self.hgt[:-2]) <= 76)
                    ),
                    self.hcl and re.match(r"^\#[0-9a-f]{6}$", self.hcl),
                    self.ecl
                    and self.ecl in ("amb", "blu", "brn", "gry", "grn", "hzl", "oth"),
                    self.pid and re.match(r"^[0-9]{9}$", self.pid),
                ]
            )
        except Exception as ex:
            breakpoint()


current_path = pathlib.Path(__file__).parent

with open(current_path / "../data/04.txt", "r") as f:
    lines = f.read()


passports = lines.split("\n\n")
passports2 = (pp.split() for pp in passports)
passports3 = ([field.split(":") for field in pp] for pp in passports2)
pp_dicts = (dict(pp) for pp in passports3)
pp_objs = (Passport(**pp) for pp in pp_dicts)
valid_pps = (obj for obj in pp_objs if obj.is_valid2())
print(len(list(valid_pps)))
