from pathlib import Path
import unittest

from subsetter_tool import get_subset_font_path


class TestSubsetterToolMethods(unittest.TestCase):
    def test_get_subset_font_path_returns_expected_output_for_valid_input(self):
        # arrange
        input_path = Path("./league-spartan-v11-latin/league-spartan-v11-latin-600.ttf")
        hash = "5fba6996"
        format = "ttf"

        # act
        result = get_subset_font_path(input_path, hash, format)

        # assert
        expected = Path("league-spartan-v11-latin-600__subset_5fba6996.ttf")
        self.assertEqual(result, expected)


if __name__ == "__main__":
    unittest.main()
