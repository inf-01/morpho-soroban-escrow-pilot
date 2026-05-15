function approveMilestone() {
    const btn = document.querySelector('.btn-primary');
    const statusBadge = document.querySelectorAll('.status-badge')[1]; // El del Hito 2
    const balanceEl = document.getElementById('total-balance');
    const milestoneItem = document.querySelectorAll('.milestone-item')[1];

    // Simular carga de transacción
    btn.innerHTML = '<span class="loader"></span> Procesando en Stellar...';
    btn.style.opacity = '0.7';
    btn.disabled = true;

    setTimeout(() => {
        // Actualizar UI
        statusBadge.innerText = 'Pagado';
        statusBadge.className = 'status-badge status-paid';
        milestoneItem.style.borderColor = 'var(--success)';
        
        // Actualizar Balance (100k - 40k = 60k)
        balanceEl.innerText = '$60,000.00';
        balanceEl.style.color = var(--success);

        // Notificación
        showNotification("¡Transacción Exitosa! Pago de $40,000 liberado.");
        
        btn.innerHTML = 'Hito Completado';
        btn.style.background = 'var(--success)';
    }, 2000);
}

function showNotification(msg) {
    const notify = document.createElement('div');
    notify.style.cssText = `
        position: fixed;
        bottom: 2rem;
        right: 2rem;
        background: var(--success);
        color: white;
        padding: 1rem 2.5rem;
        border-radius: 12px;
        box-shadow: 0 10px 30px rgba(40, 199, 111, 0.4);
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
        border: 2px solid #FFF;
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
