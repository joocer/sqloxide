# -*- coding: utf-8 -*-
from setuptools import setup

packages = \
['sqlo2']

package_data = \
{'': ['*']}

setup_kwargs = {
    'name': 'sqlo2',
    'version': '0.25.0',
    'description': 'Python bindings for sqlparser-rs',
    'long_description': open('readme.md').read(),
    'long_description_content_type': 'text/markdown',
    'author': 'Justin Joyce',
    'author_email': 'justin.joyce@joocer.com',
    'maintainer': None,
    'maintainer_email': None,
    'url': 'https://github.com/joocer/sqloxide',
    'packages': packages,
    'package_data': package_data,
    'python_requires': '>=3.6.0,<4.0',
}
from build import *
build(setup_kwargs)

setup(**setup_kwargs)
