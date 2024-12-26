# Ansible Role: Nuclei Scanner

This Ansible Galaxy role installs and configures the [Nuclei Scanner](https://github.com/projectdiscovery/nuclei), a powerful tool for vulnerability scanning and configuration management.

## Requirements

- Supported Operating Systems: Linux (e.g., Ubuntu, CentOS) , Windows and Mac 
- Dependencies:
  - `unzip` package must be available on the system.

## Role Variables

Available variables are listed below, along with their default values:

```yaml
nuclei_download_url: "https://github.com/projectdiscovery/nuclei/releases/latest/download/nuclei.zip"
nuclei_install_path: "/usr/local/bin"
nuclei_data_path: "/opt/nuclei-templates"
```

## Dependencies

This role has no specific dependencies.

## Example Playbook

```yaml
---
- name: Install and run nuclei
  hosts: target-machines
  become: true
  roles:
    - nuclei

```

for `ansible.cfg`

```ini
[default]
inventory   = ./inventory.ini
remote_user = root
log_path    = ./log/ansible.log

[privilege_escalation]
become = true
become_user = root
become_method = sudo
```

for `inventory.ini` file
```ini
[target-machines]
192.168.56.10 ansible_user=root
192.168.56.11 ansible_user=root
```

## Role Tasks

### 1. Download and Install Nuclei Binary

- Downloads the Nuclei binary ZIP file from the specified URL.
- Extracts the binary to the defined installation path.
- Ensures the installation directory exists.

### 2. Download and Update Templates

- Downloads the latest Nuclei templates to the defined data path.
- Updates templates if they already exist.

### 3. Run Nuclei Scanner

- Provides a simple wrapper for running the `nuclei` binary with specific options.

## Example Command

After installing the role, you can run Nuclei as follows:

```bash
nuclei -u http://example.com -t /opt/nuclei-templates
```

## License

MIT

## Author Information

This role was created by DannyRavi.

