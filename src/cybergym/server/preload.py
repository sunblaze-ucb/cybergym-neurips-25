import argparse
import sys
from concurrent.futures import ThreadPoolExecutor, as_completed

import docker

client = docker.from_env()


def read_tags(file_path):
    tags = []
    try:
        with open(file_path) as f:
            for line in f:
                line = line.strip()
                if not line or line.startswith("#"):
                    continue
                tags.append(f"{line}-vul")
                tags.append(f"{line}-fix")
    except FileNotFoundError:
        print(f"Error: tags file '{file_path}' not found.", file=sys.stderr)
        sys.exit(1)
    return tags


def pull_images(repo, tags, max_workers=1):
    def _pull(tag):
        image = f"{repo}:{tag}"
        print(f"Pulling {image}...")
        try:
            client.images.pull(repo, tag=tag)
            print(f"Successfully pulled {image}")
        except docker.errors.APIError as e:
            print(f"Failed to pull {image}: {e}")

    if max_workers == 1:
        for tag in tags:
            _pull(tag)
        return

    with ThreadPoolExecutor(max_workers=max_workers) as executor:
        futures = {executor.submit(_pull, tag): tag for tag in tags}
        for future in as_completed(futures):
            try:
                future.result()
            except Exception as e:
                tag = futures[future]
                print(f"Unexpected error pulling {repo}:{tag}: {e}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Pull multiple Docker images for a repo based on a list of tags.")
    parser.add_argument("--repo", "-r", default="n132/arvo", help="Docker repository (default: n132/arvo)")
    parser.add_argument(
        "--ids-file", "-f", default="arvo-ids.txt", help="Path to file containing tags (default: arvo-ids.txt)"
    )
    parser.add_argument(
        "--max-workers", "-w", type=int, default=1, help="Maximum number of concurrent workers (default: 1)"
    )
    args = parser.parse_args()

    tags = read_tags(args.ids_file)
    if not tags:
        print(f"No ids found in '{args.ids_file}'.", file=sys.stderr)
        sys.exit(1)

    pull_images(args.repo, tags, args.max_workers)
