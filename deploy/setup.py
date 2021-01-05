from setuptools import setup

setup(name='arcrust',
      version='0.1.0',
      description='Thin Python wrapper for implementing Geoprocessing Tools using Rust.',
      url='https://github.com/esride-jts/arc-rs',
      author='Jan Tschada',
      author_email='j.tschada@esri-de',
      packages=['arcrust'],
      classifiers=[
        'Programming Language :: Python :: 3',
        'License :: OSI Approved :: GNU Lesser General Public License v3 (LGPLv3)',
        'Operating System :: OS Independent'
      ],
      include_package_data=True,
      zip_safe=False)