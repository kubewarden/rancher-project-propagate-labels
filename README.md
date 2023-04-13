> **Note:** this is a context aware policy that requires Kubewarden 1.6

Rancher Manager has the concept of Project. A Project can hold many
regular Kubernetes Namespaces.

This policy monitors the creation and update of of Namespace objects and,
when they belong to a Project, it ensures a list of labels defined on the
Project are propagated to the Namespace.

The labels defined on the Project have precedence over the ones defined
inside of the Namespace.

On the Project, only the labels that start with the `propagate.` prefix
are propagated to its Namespaces. The `propagate.` prefix is stripped when
the copy operation is performed.

Namespaces that do not belong to a Rancher Project are ignored by this policy.

## Cluster access

The policy requires access to the `management.cattle.io/projects`
custom resources.

The policy requires only `GET` access to Projects defined inside of the
`local` Namespace.

Given the following assumptions:

* Kubewarden is deployed inside of the `kubewarden` Namespace
* The policy is hosted by a PolicyServer that uses the `policy-server` ServiceAccount

The following snippet will allow the policy to operate:

```yaml
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  name: rancher-project-reader
  namespace: local
rules:
- apiGroups: ["management.cattle.io"]
  resources: ["projects"]
  verbs: ["get"]
---
# Allows the policy-server ServiceAccount defined inside of the
# kubewarden namespace to read Project resources defined inside
# of the `local` namespace.
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
  name: read-rancher-projects-local
  namespace: local
subjects:
- kind: ServiceAccount
  name: policy-server
  namespace: kubewarden
roleRef:
  kind: Role
  name: rancher-project-reader
  apiGroup: rbac.authorization.k8s.io
```

## Examples

Given a Project that defines the following labels:

* `propagate.kubewarden-profile` with value `strict`
* `cost-center` with value `123`

The creation of a Namespace without labels would be changed to ensure
these labels are defined:

* `kubewarden-profile` with value `strict`

The creation of a Namespace that has the following labels:

* `kubewarden-profile` with value `low`
* `team` with value `hacking`

Would be changed by the policy to ensure these labels are defined:

* `kubewarden-profile` with value `strict`
* `team` with value `hacking`

## Limitations

Currently the policy works only when deployed inside of the `local` Rancher cluster.
That's because the Project resources are defined only inside of the cluster where
Rancher Manager is running.

When deployed to a downstream cluster, the policy isn't currently capable of
querying the upstream Project that is being referenced by the Namespace.

This limitation is going to be addressed by future releases.

## Settings

This policy has one configuration value `downstream_cluster_failure_mode`. This
defines what the policy should do when it's being deployed into a downstream
cluster.

As explained before, the policy cannot evaluate Namespace defined inside of a
downstream cluster. This configuration has two possible values:

* `ignore`: accept the Namespace CREATE/UPDATE event. This is the default value
* `fail`: reject the Namespace CREATE/UPDATE event

For example, given the following configuration:

```yaml
downstream_cluster_failure_mode: fail
```

The policy would reject the creation and update of Namespace when its being
deployed inside of a downstream cluster.

On the other hand, given this configuration:

```yaml
downstream_cluster_failure_mode: ignore
```

The creation/update of Namespace resources would always be allowed inside of
downstream clusters.
