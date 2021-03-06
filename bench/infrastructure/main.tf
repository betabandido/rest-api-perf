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
      image = "debian-cloud/debian-10"
    }
  }

  network_interface {
    network = "default"

    access_config {
      // Ephemeral IP
    }
  }

  metadata_startup_script = <<-EOT
    sudo apt-get -y update
    sudo apt-get -y install golang git tmux
    EOT

  service_account {
    scopes = [
      # default scope
      # (see https://cloud.google.com/sdk/gcloud/reference/alpha/compute/instances/set-scopes#--scopes)
      "https://www.googleapis.com/auth/devstorage.read_only",
      "https://www.googleapis.com/auth/logging.write",
      "https://www.googleapis.com/auth/monitoring.write",
      "https://www.googleapis.com/auth/pubsub",
      "https://www.googleapis.com/auth/service.management.readonly",
      "https://www.googleapis.com/auth/servicecontrol",
      "https://www.googleapis.com/auth/trace.append"
    ]
  }
}

output "instance_ip" {
  value = google_compute_instance.instance.network_interface.0.access_config.0.nat_ip
}
