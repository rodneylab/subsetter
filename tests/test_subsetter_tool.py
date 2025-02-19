from pathlib import Path
import unittest

from subsetter_tool import get_subset_font_path, write_subset_font


class TestSubsetterToolMethods(unittest.TestCase):
    def test_get_subset_font_path_returns_expected_output_for_valid_input(self):
        # arrange
        input_path = Path(
            "./fixtures/league-spartan-v11-latin/league-spartan-v11-latin-600.ttf"
        )
        hash = "5fba6996"
        format = "ttf"

        # act
        result = get_subset_font_path(input_path, hash, format)

        # assert
        expected = Path("league-spartan-v11-latin-600__subset_5fba6996.ttf")
        self.assertEqual(result, expected)

    def test_write_subset_font_returns_expected_result(self):
        # arrange
        input_path = Path(
            "./fixtures/league-spartan-v11-latin/league-spartan-v11-latin-600.woff2"
        )
        text = "Lorem ipsum dolor sit amet"
        hash = "9e5cab01"
        format = "woff2"

        # act
        result = write_subset_font(input_path, text, hash, format)

        # assert
        expected = "league-spartan-v11-latin-600__subset_9e5cab01.woff2"
        self.assertEqual(result, expected)

    def test_write_subset_font_returns_exception_when_input_path_does_not_exist(self):
        # arrange
        input_path = Path("./fixtures/does-not-exist.ttf")
        text = "Lorem ipsum dolor sit amet"
        hash = "9e5cab01"
        format = "woff2"

        # act/assert
        with self.assertRaises(FileNotFoundError):
            write_subset_font(input_path, text, hash, format)


if __name__ == "__main__":
    unittest.main()
