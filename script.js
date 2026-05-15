function approveMilestone() {
    const btn = document.querySelector('.btn-primary');
    const statusBadge = document.getElementById('status-2');
    const balanceEl = document.getElementById('total-balance');
    const hitoItem = document.querySelectorAll('.milestone-item')[1];

    // Simular carga de transacción
    btn.innerHTML = '<span class="loader"></span> Procesando en Stellar...';
    btn.style.opacity = '0.7';
    btn.disabled = true;

    setTimeout(() => {
        // Actualizar UI
        if (statusBadge) {
            statusBadge.innerText = 'Pagado';
            statusBadge.className = 'status-badge status-paid';
        }
        if (hitoItem) {
            hitoItem.style.borderColor = 'var(--success)';
        }

        // Actualizar Balance
        if (balanceEl) {
            balanceEl.innerText = '$60,000.00';
            balanceEl.style.color = 'var(--success)';
        }

        // Notificación
        showNotification("¡Transacción Exitosa! Pago de $40,000 liberado.");

        btn.innerHTML = 'Hito Completado';
        btn.style.background = 'var(--card-bg)';
        btn.style.color = 'var(--success)';
        btn.style.border = '1px solid var(--success)';
    }, 2000);
}

function showNotification(msg) {
    const notify = document.createElement('div');
    notify.style.cssText = `
        position: fixed;
        bottom: 2rem;
        right: 2rem;
        background: var(--primary);
        color: #000;
        padding: 1rem 2.5rem;
        border-radius: 12px;
        box-shadow: 0 10px 30px rgba(148, 193, 31, 0.4);
        animation: slideIn 0.5s ease-out;
        z-index: 1000;
        font-weight: 600;
    `;
    notify.innerText = msg;
    document.body.appendChild(notify);

    setTimeout(() => notify.remove(), 4000);
}

// Estilos extra para la animación de carga
const style = document.createElement('style');
style.innerHTML = `
    @keyframes slideIn {
        from { transform: translateY(100px); opacity: 0; }
        to { transform: translateY(0); opacity: 1; }
    }
    .loader {
        width: 14px;
        height: 14px;
        border: 2px solid #000;
        border-bottom-color: transparent;
        border-radius: 50%;
        display: inline-block;
        animation: rotation 1s linear infinite;
        margin-right: 8px;
    }
    @keyframes rotation {
        0% { transform: rotate(0deg); }
        100% { transform: rotate(360deg); }
    }
`;
document.head.appendChild(style);
