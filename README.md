# Free Guessing

## Generating Dataset

Panorama locations are derived from [this dataset](https://huggingface.co/datasets/stochastic/random_streetview_images_pano_v0.0.2) which was scraped from [randomstreetview.com](https://randomstreetview.com).
To generate it, save `.parquet` files from [this site](https://huggingface.co/datasets/stochastic/random_streetview_images_pano_v0.0.2/tree/main/data) and run them with the rust script in `migration/`, which will insert it into data/locations.db3.