from setuptools import setup, find_packages
from setuptools_rust import Binding, RustExtension

print(find_packages())

setup(
  name="hello",
  version="1.0",
  rust_extensions=[
    RustExtension("hello.ext", binding=Binding.PyO3)
  ],
  packages=find_packages(),
  zip_safe=False,
)
