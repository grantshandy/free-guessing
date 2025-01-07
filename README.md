# Free Guessing

## Generating Dataset

Panorama locations are derived from [this dataset](https://huggingface.co/datasets/stochastic/random_streetview_images_pano_v0.0.2), scraped from [randomstreetview.com](https://randomstreetview.com).
To generate it, save the `.parquet` files from [this site](https://huggingface.co/datasets/stochastic/random_streetview_images_pano_v0.0.2/tree/main/data) in `data/` and run the rust script in `migration/`, which will generate `public/locations.sqlite` and `src/countries.json`.