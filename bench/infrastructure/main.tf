provider "google" {
  project = "product-catalog-237219"
  region  = "europe-west6"
}

resource "google_compute_instance" "instance" {
  name         = "perf-test"
  machine_type = "n1-standard-8"
  zone         = "europe-west6-a"

  labels = {
    team = "orwell"
  }

  boot_disk {
    initialize_params {
      image = "debian-cloud/debian-9"
    }
  }

  network_interface {
    network = "default"
  }

  service_account {
    scopes = ["default"]
  }
}