import gsap from 'gsap';
import { Flip } from 'gsap/Flip';

gsap.registerPlugin(Flip);

export { Flip };
export const PANEL_MS = 260;
export const BACKDROP_MS = 320;

let backdropTween = null;

export function animatePanelLeave(panel) {
	return new Promise((resolve) => {
		gsap.to(panel, {
			yPercent: -30,
			autoAlpha: 0,
			zIndex: 8,
			duration: PANEL_MS / 1000,
			ease: 'power2.inOut',
			onComplete: resolve
		});
	});
}

export function animatePanelEnter(panel) {
	return new Promise((resolve) => {
		gsap.fromTo(
			panel,
			{ yPercent: -110, autoAlpha: 0 },
			{
				yPercent: 0,
				autoAlpha: 1,
				zIndex: 12,
				duration: PANEL_MS / 1000,
				ease: 'power3.out',
				onComplete() {
					gsap.set(panel, { clearProps: 'zIndex' });
					resolve();
				}
			}
		);
	});
}

export function animateBackdrop(backdropEl, isOpen) {
	if (!backdropEl) return;
	backdropTween?.kill();

	if (isOpen) {
		gsap.set(backdropEl, { pointerEvents: 'auto', display: 'block' });
		backdropTween = gsap.to(backdropEl, {
			autoAlpha: 1,
			duration: BACKDROP_MS / 1000,
			ease: 'power2.out'
		});
	} else {
		backdropTween = gsap.to(backdropEl, {
			autoAlpha: 0,
			duration: BACKDROP_MS / 1000,
			ease: 'power2.inOut',
			onComplete() {
				gsap.set(backdropEl, { pointerEvents: 'none', display: 'none' });
			}
		});
	}
}
