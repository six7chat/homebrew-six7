class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.30"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.30"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.30/six7-0.7.30-arm64-apple-darwin.tar.gz"
      sha256 "89993268c69ef1d72bd733cf3a6457e3ddefc7c7892b252d6fb4bd3272f91e6f"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
