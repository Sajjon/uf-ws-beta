import unittest

from one import *
from two import *

class TestCoverall(unittest.TestCase):

    def test(self):
        self.assertEqual(beta_record_to_object(new_beta_record_default()), BetaObject.new_default())
                         
if __name__=='__main__':
    unittest.main()