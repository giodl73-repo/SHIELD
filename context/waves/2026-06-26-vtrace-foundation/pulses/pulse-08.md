# Pulse 08: ARCHITECTURE

Settled. Authored `docs/vtrace/ARCHITECTURE.md`: 7 components (shield-network / corpus / score /
tier / gap / cli + docs review layer), scale allocated to the corpus and gap layers, downward-only
dependency direction, data flow, dependencies, and failure modes (incl. demand-basis-unknown, transfer-semantics risk, and
public-source proxy-as-proof → hold/label). Fixed point: removed a potential `corpus→score` cycle. Next:
INTERFACES.
