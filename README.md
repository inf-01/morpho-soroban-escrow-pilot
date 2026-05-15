# Smart Escrow CRJungle | Morpho Studio Pilot

Solución institucional de Escrow inteligente para la gestión de contratos públicos de alto impacto ($100,000 USD) sobre la red **Stellar (Soroban)**.

## 🚀 Propósito
Este proyecto demuestra cómo un contrato inteligente puede garantizar la reserva presupuestaria y la transparencia en la ejecución de hitos para la conservación de la biodiversidad, integrando roles institucionales y validación de evidencia on-chain.

## 🏗️ Arquitectura del Proyecto

### 1. Smart Contract (Blockchain)
Ubicado en `/contracts/src/lib.rs`.
- **Tecnología:** Rust / Soroban SDK.
- **Lógica de Hitos:** Gestión de 3 hitos ($20k, $40k, $40k).
- **Gobernanza:** Implementa roles de `Supervisor` (aprobación técnica) y `Financiero` (liberación de fondos).
- **Seguridad:** Uso de `require_auth` para control de acceso.

### 2. Operational Dashboard (Interfaz)
Ubicado en `/dashboard/`.
- **Tecnología:** HTML5, CSS3 (Glassmorphism), Vanilla JavaScript.
- **Branding:** CRJungle Protocol (Biodiversidad Digital).
- **Funcionalidad:** Visualiza el estado real de los hitos y simula la interacción con el contrato inteligente.

## 🧪 Flujo de Operación
1. **Inicialización:** El contrato define los roles y bloquea los fondos del token correspondiente.
2. **Entrega de Evidencia:** El proveedor sube evidencia técnica (maqueada en el dashboard).
3. **Aprobación Técnica:** El Supervisor valida el hito on-chain.
4. **Liberación de Fondos:** El Financiero ejecuta la transferencia automática mediante el Smart Contract.

## 👥 Roles Definidos
- **Institución:** Dueño de los fondos.
- **Proveedor:** Ejecutor del proyecto.
- **Supervisor:** Validador de la ciencia y técnica.
- **Financiero:** Autorizador del desembolso.
- **Árbitro:** Resolutor de disputas críticas.

---
**Desarrollado para Morpho Studio | CRJungle Protocol**
