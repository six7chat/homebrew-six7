class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.1 - powered by Korium 0.7.8"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.8"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.8/six7-0.7.8-arm64-apple-darwin.tar.gz"
      sha256 "37e9bf196e485e1ce889e80d050f9be2c796f10a4e994d0baa95da2875b1c662"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
